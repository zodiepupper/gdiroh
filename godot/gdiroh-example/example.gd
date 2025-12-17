extends Control

const EXAMPLE_ALPN = "/gdiroh/example/0"

@onready var _address_text_edit = $VBoxContainer/TextEdit

var _endpoint: IrohEndpoint

@onready var local_address: Label = %local_address
@onready var copy_local_address: Button = %copy_local_address

func _init() -> void:
	_endpoint = IrohEndpoint.new()
	_endpoint.accept_async_result.connect(_on_peer_accepted)
	_endpoint.connect_async_result.connect(_on_peer_connected)
	_endpoint.bind_async_result.connect(_on_endpoint_bound)
	
	_endpoint.bind_async([EXAMPLE_ALPN])
	

func _ready() -> void:
	local_address.text = _endpoint.address()
	copy_local_address.pressed.connect(_copy_local_address)

func _copy_local_address() -> void:
	DisplayServer.clipboard_set(_endpoint.address())

func _on_endpoint_bound(result: bool):
	if result:
		var key = _endpoint.address()
		print("Endpoint binded! \\(>.<)/")
		print("My address: `", key, "`")
	else:
		print("Endpoint did not bind! ;-;")
	

func _on_peer_accepted(connection: IrohConnection):
	print("Peer accepted! ", connection)
	var stream = connection.accept_bi_blocking()
	print(stream)
	

func _on_peer_connected(connection: IrohConnection):
	print("Peer connected! ", connection)
	var stream = connection.open_bi_blocking()
	print(stream)

func _on_serve_pressed() -> void:
	_endpoint.accept_async()

func _on_connect_pressed() -> void:
	var key = _address_text_edit.text.strip_edges()
	_endpoint.connect_async(key, EXAMPLE_ALPN)
