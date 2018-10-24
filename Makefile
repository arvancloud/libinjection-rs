clean:
	rm -rf libinjection

fix-python:
	sed -i 's/python$$/python2/g' libinjection/src/*.py

.PHONY: clean fix-python
