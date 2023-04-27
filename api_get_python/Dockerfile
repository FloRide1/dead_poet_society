FROM python:3.8-slim

WORKDIR /app

COPY requirements.txt requirements.txt
RUN pip install -r requirements.txt

COPY main.py .
EXPOSE 5001

CMD ["python", "main.py"]
