clean:
	rm -rf libinjection
	rm -f src/bindings.rs
	cargo clean

fix-python:
	sed -i 's/python$$/python2/g' libinjection/src/*.py

.PHONY: clean fix-python
