extends Control


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func _on_slider_value_changed(value):
	$Value.set_text(str(value))