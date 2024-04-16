extends Control

func _ready():
	get_tree().get_root().files_dropped.connect(_on_files_dropped)
	_load_image("res://icon.svg")

func _on_load_image_button_down():
	$UI/FileDialog.popup()
	

func _on_file_dialog_file_selected(path):
	_load_image(path)

func _on_files_dropped(files):
	var path = files[0]	
	_load_image(path)
	
func _load_image(path):
	var image = Image.new()
	image.load(path)
	
	$UI/AsciiButton.set_pressed(false)
	Utils.fit_image_to_size(image, $ColorRect.get_rect().size.x, $ColorRect.get_rect().size.y);
	$ColorRect/ImageRect.set_image(image)

#func _on_ascii_button_toggled(toggled_on):
	#if (toggled_on):
		#$ColorRect/ImageRect.convert_to_ascii()
	#else:
		#$ColorRect/ImageRect.reset_texture()
		#

func set_font(font: Font, font_size: int):
	$ColorRect/ImageRect.set_font(font, font_size)

func _on_slider_value_changed(value):
	set_font(preload("res://COUR.TTF"), value)
