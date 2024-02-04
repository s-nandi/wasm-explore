from abc import abstractmethod
from typing import Protocol

class HostNetworkProvider(Protocol):
    @abstractmethod
    def get(self, uri: str, filename: str) -> None:
        raise NotImplementedError

