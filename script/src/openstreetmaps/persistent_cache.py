import csv
import json
import os
import time
from functools import wraps


def persistent_cache(csv_file="function_cache.csv"):
    """A decorator that caches function results in a CSV file.

    Works with any number of arguments that can be converted to strings.
    Arguments are stored as a single JSON string in the cache file.
    """

    # Load the cache if it exists
    cache = {}
    if os.path.exists(csv_file):
        try:
            with open(csv_file, "r", newline="") as f:
                reader = csv.reader(f)
                headers = next(reader)  # Skip the header row
                for row in reader:
                    if len(row) != 2:  # Should have args_key and result columns
                        print(f"Warning: Invalid cache row: {row}")
                        continue

                    args_key, result = row

                    # Convert result back to float/int if possible
                    try:
                        result = float(result)
                        if result.is_integer():
                            result = int(result)
                    except ValueError:
                        # If not a number, keep as string
                        pass

                    cache[args_key] = result

        except Exception as e:
            print(f"Error loading cache: {e}")
    else:
        # Create new file with header if it doesn't exist
        with open(csv_file, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(["args_key", "result"])

    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            # Create a cache key from all arguments
            # For simplicity, serialize to JSON
            if kwargs:
                cache_key = json.dumps((args, kwargs), sort_keys=True)
            else:
                cache_key = json.dumps(args, sort_keys=True)

            # Check if result is in cache
            if cache_key in cache:
                return cache[cache_key]

            # Call the function and save result to cache
            result = func(*args, **kwargs)
            cache[cache_key] = result

            # Append just the new entry to CSV
            try:
                with open(csv_file, "a", newline="") as f:
                    writer = csv.writer(f)
                    writer.writerow([cache_key, result])
            except Exception as e:
                print(f"Error appending to cache: {e}")

            return result

        return wrapper

    return decorator
