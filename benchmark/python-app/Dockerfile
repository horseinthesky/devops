FROM mwalbeck/python-poetry:1.7.1-3.12 as deps
WORKDIR /app
COPY pyproject.toml .
RUN : \
  && poetry config virtualenvs.create false \
  && poetry export \
    --without-hashes \
    --format requirements.txt \
    --output requirements.txt \
  && pip install \
    --no-cache-dir \
    --disable-pip-version-check \
    --root-user-action=ignore \
    -r requirements.txt \
    -t /bundle/

FROM python:3.12.2-slim-bookworm
COPY --from=deps /bundle/ /usr/local/lib/python3.12/site-packages/
WORKDIR /app
COPY . .
ENV HOST=0.0.0.0 PORT=8000 PYTHONDONTWRITEBYTECODE=1
ENTRYPOINT ["sh", "-c", "/usr/local/lib/python3.12/site-packages/bin/uvicorn --loop uvloop --host $HOST --port $PORT app:app"]
