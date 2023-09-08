from demo import exports
from demo.types import Err
from demo.exports.greeter import Greeting
from demo.imports.greeter_candidates import Person, CandidateHermit, CandidateExcited, get

class Greeter(exports.Greeter):
    def greet(self) -> Greeting:
        for candidate in get():
            if isinstance(candidate, CandidateExcited) and candidate.value.lego_count is not None and candidate.value.lego_count > 0:
                return Greeting(f"Hello, {candidate.value.name}!", candidate.value.lego_count)

        raise Err("no suitable candidate found")
                    
