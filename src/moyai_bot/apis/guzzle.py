import requests

GUZZLE: str = "https://guzzle.gay/api"


def get_random_teawie() -> str:
	resp: requests.Response = {}
	try:
		resp = requests.get(GUZZLE + "/get_random_teawie", timeout=30)
	except (requests.RequestException, requests.ConnectionError,
	        requests.HTTPError, requests.JSONDecodeError):
		return "something went wrong :("
	if not resp.status_code == 200:
		return "api request failed :("

	try:
		ret = resp.json()["url"]
	except KeyError:
		return "couldn't get url from api response :("
	return ret
