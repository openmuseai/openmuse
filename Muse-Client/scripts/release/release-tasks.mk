# Local stage entry points. Store actions remain in the individual runbooks.
SHELL := /bin/bash
.SHELLFLAGS := -euo pipefail -c
RUN ?= docs/release/runs/next-release
VERSION ?=
ANDROID_BUILD ?=
IOS_BUILD ?=
ARCHIVE ?= dist/release-archives/next-release/submission-01.zip

.PHONY: plan check test-mobile verify-guide gate-build build-play build-ios-us build-cn build-ios-cn gate-upload gate-submit gate-publish archive verify-archive

plan:
	python3 scripts/release/pipeline.py plan --run "$(RUN)"

check:
	python3 scripts/release/pipeline.py check --run "$(RUN)"

.PHONY: preflight-macos
preflight-macos:
	python3 scripts/release/preflight-macos.py --app dist/OpenMuse.app --report "$(RUN)/evidence/preflight-macos.json"

test-mobile:
	cd app/openmuse_mobile && flutter analyze && flutter test

verify-guide:
	python3 scripts/release/verify-welcome-guide.py

gate-build:
	python3 scripts/release/pipeline.py check --run "$(RUN)" --gate 05-build

build-play: gate-build
	bash scripts/release/build-android-store.sh --lane play --version "$(VERSION)" --build-number "$(ANDROID_BUILD)"

build-ios-us: gate-build
	bash scripts/release/build-ios-store.sh --market us --version "$(VERSION)" --build-number "$(IOS_BUILD)"

build-cn: gate-build
	bash scripts/release/build-android-store.sh --lane cn --version "$(VERSION)" --build-number "$(ANDROID_BUILD)"

build-ios-cn: gate-build
	bash scripts/release/build-ios-store.sh --market cn --version "$(VERSION)" --build-number "$(IOS_BUILD)"

gate-upload:
	python3 scripts/release/pipeline.py check --run "$(RUN)" --gate 08-upload

gate-submit:
	python3 scripts/release/pipeline.py check --run "$(RUN)" --gate 09-submit

gate-publish:
	python3 scripts/release/pipeline.py check --run "$(RUN)" --gate 11-publish

archive:
	python3 scripts/release/pipeline.py archive --run "$(RUN)" --output "$(ARCHIVE)"

verify-archive:
	python3 scripts/release/pipeline.py verify-archive "$(ARCHIVE)"
