from typing import TypeVar, Generic, Union, Optional, Union, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


class Generator(Protocol):

    @abstractmethod
    def generate(self) -> int:
        raise NotImplementedError

    @abstractmethod
    def generate_many(self) -> List[int]:
        raise NotImplementedError


