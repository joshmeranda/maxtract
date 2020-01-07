VENV_BIN=venv/bin

MODULES=error extract mapper

.PHONy: test single-test clean

build:
	python setup.py -v build

install:
	python setup.py -v install

single-test:
	source ${VENV_BIN}/activate; \
	python -m unittest -v ${TEST}

test:
	source ${VENV_BIN}/activate; \
	python -m unittest -v

lint:
	pylint --verbose ${MODULES}

clean:
	rm --verbose --recursive --force build dist
