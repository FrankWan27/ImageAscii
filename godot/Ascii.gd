extends Ascii

func _ready():
	self.initialize($SubViewportContainer/CharViewport/Draw, $SubViewportContainer/CharViewport)

func _on_ascii_button_pressed():
	self.capture_viewport().save_png("res://test2.png")


