#instructions to create docker container from the docker image of the agent. 
# syntax=docker/dockerfile:1.4
FROM alpine
COPY --link /foo /bar

RUN dnf install -y \
        python3.11 \
        python3-pip \
        python3.11-devel \


COPY requirements.txt .
RUN rm requirements.txt
