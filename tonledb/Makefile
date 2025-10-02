IMAGE ?= ghcr.io/$(shell echo $$GITHUB_REPOSITORY_OWNER 2>/dev/null || echo "your-org")/tonledb:latest

.PHONY: build build-all build-cli build-examples image sbom sign verify scan k8s-apply k8s-delete docker-hardened

build:
	cargo build -p tonledb-network --release

build-all:
	cargo build --release --workspace

build-cli:
	cargo build -p tonledb-cli --release

build-examples:
	cargo build -p tonledb-examples --release

image:
	docker build -t $(IMAGE) .

sbom:
	syft $(IMAGE) -o cyclonedx-json > sbom.cdx.json

sign:
	cosign sign --yes $(IMAGE)

verify:
	cosign verify $(IMAGE)

scan:
	trivy image --severity HIGH,CRITICAL $(IMAGE)

k8s-apply:
	kubectl apply -f k8s/security/podsecurity.yaml
	kubectl apply -f k8s/security/networkpolicy.yaml
	kubectl apply -f k8s/security/deployment.yaml

k8s-delete:
	kubectl delete -f k8s/security/deployment.yaml --ignore-not-found
	kubectl delete -f k8s/security/networkpolicy.yaml --ignore-not-found
	kubectl delete -f k8s/security/podsecurity.yaml --ignore-not-found

docker-hardened:
	@echo "Load AppArmor once: sudo apparmor_parser -r -W apparmor/tonledb.apparmor"
	@echo "Run:"
	@echo "  docker run --rm -p 8383:8383 \\"
	@echo "    --security-opt seccomp=$(PWD)/k8s/security/seccomp.json \\"
	@echo "    --security-opt apparmor=tonledb \\"
	@echo "    --read-only --cap-drop ALL \\"
	@echo "    -e TLDB_KEK=$$(openssl rand -base64 32) \\"
	@echo "    $(IMAGE)"