repo_name := github-issues
version := v$(shell cat Cargo.toml | grep version | cut -d '"' -f2)
artifact_osx = $(repo_name)-$(version)-osx-amd64.tar.gz

OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include

test:
	@cargo test
.PHONY: test

debug:
	OPENSSL_INCLUDE_DIR=$(OPENSSL_INCLUDE_DIR) cargo build
.PHONY: debug

fetch:
	./target/debug/github-issues fetch arnau/test \
      --oauth-token $(GITHUB_TOKEN) \
      --output test.csv \
      --format csv
.PHONY: fetch

upload:
	./target/debug/github-issues upload arnau/test \
      --oauth-token $(GITHUB_TOKEN) \
      --input ./test1.csv
.PHONY: upload


build:
	@cargo build --release
.PHONY: build

release: release-create release-artifacts
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

release-info:
	github-release info --user ustwo --repo $(repo_name)
.PHONY: release-info

release-delete:
	github-release delete --user ustwo --repo $(repo_name) --tag $(version)
.PHONY: release-delete

artifacts: dist/$(artifact_osx)
.PHONY: artifacts

dist/$(artifact_osx): build
	@mkdir -p dist
	@echo "Compressing"
	@cp target/release/$(repo_name) dist/$(repo_name)
	@cp LICENSE dist/LICENSE
	@cp README.md dist/README.md
	@tar -zcvf $@ -C dist/ $(repo_name) \
                         LICENSE \
                         README.md
	@echo "****************************************************************"
	@shasum -a 256 $@
	@du -sh $@
	@echo "****************************************************************"

artifacts-expand:
	cd dist && \
	mkdir -p temp && \
	tar -zxvf $(artifact_osx) -C temp/

homebrew-create:
	brew create tar --set-name $(repo_name)

homebrew-install:
	brew install ustwo/tools/$(repo_name)

homebrew-flush:
	rm -f /Library/Cache/Homebrew/$(repo_name)*
