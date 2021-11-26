<div align="center">

  <img src="https://user-images.githubusercontent.com/45036977/142221142-db6a78ed-9fd7-4bb2-b19f-8df0a5fc8168.jpg" height="200" />

  # monke
  A statically typed functional programming language. Made with :heart: and rust

</div>
  
#### Dependency Graph

* blue -> dev dependency
* green -> build dependency (doesn't contribute code)
* black -> normal dependency

<img src="https://github.com/matievisthekat/monke/blob/master/dependency_graph.png" />
  
#### Syntax
```
func main:[] -> int32 {
  set x = 2 * 5;
  set y = x + 9;
  
  set str = "hello world";
  set char = 'd';
  
  set empty = void;
  empty = VoidToString["not empty anymore!"];
  
  set const MY_NAME = "matthew";
  MY_NAME = "john"; /* throws error */
  
  set List[5] list = []; /* definite size */
  set new_list = []; /* indefinite size */
  set name, age, height = ["matthew", 24, 183.5]; /* destructure lists */
  
  std::out::writeln["hellow world"];
  
  set list_of_args = ["hello", "world"];
  std::out::writeln[:list_of_args]; /* destructures list into function arguments. "hello" is arg 0 and "world" is arg 1 */
}
```

If you'd like to help just send me an email (`matievisthekat@gmail.com`) or open an issue
