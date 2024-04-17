extends Ascii

func _ready():
	self.initialize($SubViewportContainer/CharViewport/Draw, $SubViewportContainer/CharViewport, $SubViewportContainer)
	#self.initialize($Draw, $SubViewportContainer/CharViewport)

func _on_ascii_button_pressed():
	self.capture_viewport().save_png("res://test3.png")


