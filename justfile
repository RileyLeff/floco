generate-readme:
	cargo readme > README.md

generate-doc:
	cargo doc

prep-release:
	release-plz update
	git add . 
	git commit -m "chore: release"

run-release:
	release-plz release