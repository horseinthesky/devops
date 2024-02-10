from dataclasses import dataclass
from pathlib import Path
from typing import Self

import yaml


@dataclass
class Config:
    otlp_endpoint: str

    @classmethod
    def from_yaml(cls, path: str) -> Self:
        config = yaml.safe_load(Path(path).read_text())
        return cls(
            otlp_endpoint=config.get("otlp_endpoint"),
        )
