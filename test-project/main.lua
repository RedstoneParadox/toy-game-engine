test_object = game_objects.test_object
components = test_object.components
messenger = components[1]

print(messenger["message"])
print("Does this have newlines?")