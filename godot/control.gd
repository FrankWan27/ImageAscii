extends Control

func _ready():
	get_tree().get_root().files_dropped.connect(_on_files_dropped)

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
	
	Utils.fit_image_to_size(image, $ColorRect.get_parent().get_rect().size.x, $ColorRect.get_parent().get_rect().size.y);
	$ColorRect/ImageRect.set_image(image)

func _on_ascii_button_toggled(toggled_on):
	if (toggled_on):
		$ColorRect/ImageRect.convert_to_ascii()
	else:
		$ColorRect/ImageRect.reset_texture()
