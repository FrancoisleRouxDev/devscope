"""Business logic that sits between HTTP routes and the scanner bridge.

Services know nothing about HTTP (no ``Request``, no status codes) — they take
and return plain Python/Pydantic objects, which keeps them easy to unit-test.
"""
