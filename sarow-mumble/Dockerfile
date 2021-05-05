FROM python:3.9-alpine as builder
COPY . /sarow-mumble

RUN apk update && \
    apk upgrade && \
    apk add gcc musl-dev && \
    pip install -U "https://github.com/Nuitka/Nuitka/archive/factory.zip" && \
    pip install -r /sarow-mumble/requirements.txt && \
    nuitka3 --python-flag=no_site --python-flag=no_warnings --standalone --follow-imports /sarow-mumble/src/sarow-mumble.py

FROM alpine:edge
RUN apk update && \
    apk upgrade && \
    apk add binutils opus-dev
COPY --from=builder /sarow-mumble.dist /sarow-mumble
CMD ["/sarow-mumble/sarow-mumble"]
