fix-python:
	perl -i -pe 's/python$$/python2/g' $(OUT_DIR)/libinjection/src/*.py

.PHONY: fix-python
