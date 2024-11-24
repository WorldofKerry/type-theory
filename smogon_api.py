from enum import Enum
import requests

class Format(Enum):
    GEN9UU = "gen9uu"
    GEN9OU = "gen9ou"

class API(Enum):
    stats = "https://pkmn.github.io/smogon/data/stats/"

def get_stats(format: Format) -> dict:
    url = f"{API.stats.value}{format.value}.json"
    return requests.get(url).json()

def main():
    raw_stats = get_stats(Format.GEN9OU)

    great_tusk_count = raw_stats["pokemon"]["Great Tusk"]["count"]

    # Only care about name and count
    stats = {
        k: v["count"] for k, v in raw_stats["pokemon"].items()
    }

    print(stats)

if __name__ == "__main__":
    main()