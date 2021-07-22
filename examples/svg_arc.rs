use tagger::prelude::*;
use tagger::{attr_builder, path_builder};
fn main() {
    let width = 500.0;
    let height = 400.0;

    let mut svg = {
        let svg_attr = attr_builder()
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("viewBox", formatm!("0 0 {} {}", width, height))
            .build();

        elem!("svg", svg_attr)
    };

    let path = {
        use tagger::PathCommand::*;
        let path = path_builder()
            .add(M(200, 120))
            .add(Q(300, 50, 400, 120))
            .add(T(500, 120))
            .build();

        let gc = attr_builder()
            .attr("stroke", "black")
            .attr("stroke-width", 2)
            .attr("fill", "green")
            .attr("fill-opacity", 0.5)
            .attr_whole(path)
            .build();

        elem!("path", gc)
    };

    svg.append(path);

    let path = {
        use tagger::PathCommand::*;
        let path = path_builder()
            .add(M(300, 200))
            .add(H_(-150))
            .add(A_(150, 150, 0, 1, 0, 150, -150))
            .add(Z(""))
            .build();

        let gc = attr_builder()
            .attr("stroke", "black")
            .attr("stroke-width", 2)
            .attr("fill", "blue")
            .attr("fill-opacity", 0.5)
            .attr_whole(path)
            .build();

        elem!("path", gc)
    };

    svg.append(path);

    println!("{}", svg);
}
