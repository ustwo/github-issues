repo_name := github-issues
version := v$(shell cat Cargo.toml | grep version | cut -d '"' -f2)
artifact_osx = github-issues-$(version)-osx-amd64.tar.gz

release: release-create release-artifacts
	git tag $(version)
	# git push --tags origin $(GIT_BRANCH)
.PHONY: release

release-create:
	github-release release --user ustwo \
                         --repo $(repo_name) \
                         --tag $(version)
.PHONY: release-create

release-artifacts: artifacts
	github-release upload --user ustwo \
                        --repo $(repo_name) \
                        --tag $(version) \
                        --name $(artifact_osx) \
                        --file dist/$(artifact_osx)
.PHONY: release-artifacts

build:
	@cargo build --release
.PHONY: artifact

dist/$(artifact_osx): build
	@mkdir -p dist
	@echo "Compressing"
	@cp target/release/github-issues dist/github-issues
	@cp LICENSE dist/LICENSE
	@cp README.md dist/README.md
	@tar -zcvf $@ -C dist/ github-issues \
                         LICENSE \
                         README.md
	@echo "****************************************************************"
	@shasum -a 256 $@
	@du -sh $@
	@echo "****************************************************************"

release-expand:
	cd dist
	mkdir -p temp
	tar -zxvf $(tarball) -C temp/

release-info:
	github-release info --user ustwo --repo mastermind
.PHONY: release-info

release-delete:
	github-release delete --user ustwo --repo mastermind --tag $(version)
.PHONY: release-delete

bundle-mastermind:
	pyinstaller mastermind.spec
.PHONY: bundle-mastermind

bundle-proxyswitch:
	pyinstaller proxyswitch.spec
.PHONY: bundle-proxyswitch

homebrew-create:
	brew create tar --set-name mastermind

homebrew-install:
	brew install mastermind

homebrew-flush:
	rm -f /Library/Cache/Homebrew/mastermind*

test: docker-test
.PHONY: test

local-test: docker-local-test
.PHONY: local-test

raw-test:
	$(NOSE) -s
.PHONY: raw-test

include tasks/*.mk
