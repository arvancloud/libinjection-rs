fix-python:
	sed -i 's/python$$/python2/g' $(OUT_DIR)/libinjection/src/*.py

.PHONY: fix-python
