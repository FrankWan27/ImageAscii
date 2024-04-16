extends Control

var font = load("res://COUR.TTF")

func _draw():
	draw_string(font, Vector2(0, 0), "C", HORIZONTAL_ALIGNMENT_CENTER, 20, 30, Color.GREEN)
