from typing import TypeAlias

ScalarValue: TypeAlias = int | float | str | bool | None


class ConnectionOptions:
    host: str
    port: int
    tls: bool
    allow_insecure_tls: bool
    pat_token: str | None

    def __init__(
        self,
        host: str,
        port: int,
        tls: bool = False,
        pat_token: str | None = None,
        allow_insecure_tls: bool = False,
    ) -> None: ...
    def ws_url(self) -> str: ...


class IntegerVar:
    name: str
    unit: str | None
    min: float | None
    max: float | None

    def __init__(
        self,
        name: str,
        unit: str | None = None,
        min: float | None = None,
        max: float | None = None,
    ) -> None: ...


class FloatVar:
    name: str
    unit: str | None
    min: float | None
    max: float | None

    def __init__(
        self,
        name: str,
        unit: str | None = None,
        min: float | None = None,
        max: float | None = None,
    ) -> None: ...


class TextVar:
    name: str
    unit: str | None
    options: list[str]
    max_len: int | None

    def __init__(
        self,
        name: str,
        unit: str | None = None,
        options: list[str] = ...,
        max_len: int | None = None,
    ) -> None: ...


class BooleanVar:
    name: str
    unit: str | None

    def __init__(self, name: str, unit: str | None = None) -> None: ...


class VariableMetadataPatch:
    unit: str | None
    min: float | None
    max: float | None
    options: list[str]
    max_len: int | None

    def __init__(
        self,
        unit: str | None = None,
        min: float | None = None,
        max: float | None = None,
        options: list[str] = ...,
        max_len: int | None = None,
    ) -> None: ...


class FolderInfo:
    id: str
    name: str

    def __init__(self, id: str, name: str) -> None: ...


class VarInfo:
    id: str
    name: str
    var_d_type: str
    unit: str | None
    min: float | None
    max: float | None
    options: list[str]
    max_len: int | None

    def __init__(
        self,
        id: str,
        name: str,
        var_d_type: str,
        unit: str | None = None,
        min: float | None = None,
        max: float | None = None,
        options: list[str] = ...,
        max_len: int | None = None,
    ) -> None: ...


class Subscription:
    def next_event(self, timeout_ms: int | None = None) -> tuple[str, ScalarValue] | None: ...
    def close(self) -> None: ...


class Client:
    @staticmethod
    def connect(
        host: str,
        port: int,
        tls: bool = False,
        allow_insecure_tls: bool = False,
    ) -> Client: ...
    @staticmethod
    def connect_with_pat(
        host: str,
        port: int,
        tls: bool,
        pat_token: str,
        allow_insecure_tls: bool = False,
    ) -> Client: ...
    @staticmethod
    def connect_with_options(options: ConnectionOptions) -> Client: ...

    def connection_options(self) -> ConnectionOptions: ...
    def is_connected(self) -> bool: ...
    def disconnect(self) -> None: ...

    def list(
        self,
        folder_id: str | None = None,
        timeout_ms: int = 5000,
    ) -> tuple[list[FolderInfo], list[VarInfo]]: ...

    def create_folders(
        self,
        names: list[str],
        parent_id: str | None = None,
        timeout_ms: int = 5000,
    ) -> None: ...

    def create_integer_variables(
        self,
        vars: list[IntegerVar],
        parent_id: str | None = None,
        timeout_ms: int = 5000,
    ) -> None: ...

    def create_float_variables(
        self,
        vars: list[FloatVar],
        parent_id: str | None = None,
        timeout_ms: int = 5000,
    ) -> None: ...

    def create_text_variables(
        self,
        vars: list[TextVar],
        parent_id: str | None = None,
        timeout_ms: int = 5000,
    ) -> None: ...

    def create_boolean_variables(
        self,
        vars: list[BooleanVar],
        parent_id: str | None = None,
        timeout_ms: int = 5000,
    ) -> None: ...

    def delete_items(self, item_ids: list[str], timeout_ms: int = 5000) -> None: ...

    def get_values(self, var_ids: list[str], timeout_ms: int = 5000) -> list[ScalarValue]: ...

    def set_integer_variables(
        self,
        var_ids: list[str],
        values: list[int],
        timeout_ms: int = 5000,
    ) -> None: ...

    def set_float_variables(
        self,
        var_ids: list[str],
        values: list[float],
        timeout_ms: int = 5000,
    ) -> None: ...

    def set_text_variables(
        self,
        var_ids: list[str],
        values: list[str],
        timeout_ms: int = 5000,
    ) -> None: ...

    def set_boolean_variables(
        self,
        var_ids: list[str],
        values: list[bool],
        timeout_ms: int = 5000,
    ) -> None: ...

    def edit_variable_metadata(
        self,
        var_id: str,
        patch: VariableMetadataPatch,
        timeout_ms: int = 5000,
    ) -> None: ...

    def create_bulk_from_json(
        self,
        json: str,
        parent_id: str | None = None,
        timeout_ms: int = 5000,
    ) -> None: ...

    def subscribe_var_values(self, var_ids: list[str], timeout_ms: int = 5000) -> Subscription: ...
