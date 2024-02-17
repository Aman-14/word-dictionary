from locust import HttpUser, task, between
import random
import json

words = json.load(open("words.json"))


class MyUser(HttpUser):
    wait_time = between(1, 3)  # Wait time between requests

    @task
    def my_task(self):
        self.client.get(f"/definition?word={random.choice(words)}")
