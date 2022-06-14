# syntax=docker/dockerfile:1

FROM python:3.9

WORKDIR /app

RUN pip install --upgrade pip
COPY ./requirements.txt /app/requirements.txt
RUN pip3 install -r requirements.txt
RUN apt-get -y update
RUN apt-get -y upgrade
RUN yes | apt-get install -y ffmpeg
RUN yes | apt-get install chromium

COPY . /app

CMD [ "python3", "backend/main.py"]
