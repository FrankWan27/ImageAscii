extends Ascii

func _ready():
	self.initialize($SubViewportContainer/CharViewport/Draw, $SubViewportContainer/CharViewport, $SubViewportContainer)



func _on_invert_button_toggled(toggled_on):
	self.set_invert(toggled_on)


func _on_skip_backtick_button_toggled(toggled_on):
	self.set_skip_backtick(toggled_on)
