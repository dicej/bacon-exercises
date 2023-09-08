from hello_name import exports, who_to_greet

class Run(exports.Run):
    def run(self):
        print(f"Hello, {who_to_greet()}!")
