PKGS=error maxtract
RM=rm --recursive --verbose --force
PIP=pip3

#---------------------------------------------------------#
# Project setup and build targets                         #
#---------------------------------------------------------#

venv:
	${PIP} install --progress-bar ascii virtualenv
	python -m virtualenv venv

#---------------------------------------------------------#
# Phony project maintenance targets                       #
#---------------------------------------------------------#

.PHONY: setup fix-imports check test mostlyclean clean

setup: venv
	source venv/bin/activate; \
		${PIP} install --progress-bar ascii --requirement requirements.txt

lint:
	pylint --output-format colorized ${PKGS} setup.py

fix-imports:
	isort --combine-as \
		--combine-star \
		--lines-after-imports 2 \
		--recursive \
		${PKGS} setup.py

mostlyclean:
	find . -name __pycache__ -exec ${RM} '{}' +
	${RM} build logs

clean: mostlyclean
	${RM} venv
