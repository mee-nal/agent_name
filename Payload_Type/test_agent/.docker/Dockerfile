#instructions to create docker container from the docker image of the agent. 
FROM nginx

RUN dnf install -y \
        python3.11 \
        python3-pip \
        python3.11-devel \


COPY requirements.txt .
RUN rm requirements.txt
