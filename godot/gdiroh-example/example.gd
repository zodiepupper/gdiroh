extends Control

@onready var text_edit = $VBoxContainer/TextEdit

var alpn = "gdiroh-example"
var endpoint: IrohEndpoint

func _init() -> void:
	endpoint = IrohEndpoint.new()
	
	endpoint.accept_async_result.connect(_on_peer_connected)
	endpoint.connect_async_result.connect(_on_peer_connected)
	endpoint.bind_async_result.connect(_on_endpoint_bound)
	
	endpoint.bind_async(alpn)

func _on_endpoint_bound(result: bool):
	if result:
		var key = endpoint.addr()
		print("My addr: `", key, "`")
	else:
		print("Endpoint did not bind!")
	

func _on_peer_connected(connection: IrohConnection):
	print("Peer connected! ", connection)

func _on_serve_pressed() -> void:
	endpoint.accept_async()

func _on_connect_pressed() -> void:
	var key = text_edit.text
	endpoint.connect_async(key, alpn)
