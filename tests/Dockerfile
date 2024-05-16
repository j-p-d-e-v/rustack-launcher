FROM django:latest
RUN apt update -y

RUN mkdir -p app
WORKDIR /app
COPY /testapp/requirements.txt /tmp/requirements.txt
RUN pip install -r /tmp/requirements.txt
COPY testapp /app/

EXPOSE 80 8080 8000

ENTRYPOINT [ "sh", "/app/entrypoint.sh" ]