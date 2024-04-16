extends TextureRect

var font = load("res://COUR.TTF")
# Called when the node enters the scene tree for the first time.
func _ready():

	pass # Replace with function body.

func _draw():
	draw_string(font, Vector2(0, 0), "C", HORIZONTAL_ALIGNMENT_CENTER, -1, 30, Color.BLACK)
	pass

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	await RenderingServer.frame_post_draw
	#self.get_texture().get_image().save_png("res://test.png")
