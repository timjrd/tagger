use std::fmt::Write;
use tagger::xml2::AttrTrait;

static COMMENT:[&'static str;3]=["<!--","","-->"];
static XML_DECL:[&'static str;3]=["<?","xml","?>"];


fn main() -> core::fmt::Result {
    let mut io = tagger::upgrade(std::io::stdout());

    let mut root = tagger::xml2::Element::new(io);

    root.single(XML_DECL,|a|{a.attr("version",1.0)?.attr("encoding","UTF-8")?.attr("standalone","no")})?;

    root.single(COMMENT,|mut a|{write!(a,"{}","This is a comment")?;Ok(a)})?;

    let mut v:Vec<[f32;2]>=Vec::new();

    root.elem2("svg",|builder|{
        let svg=builder.build(|a|{
            a.attr("chicken","but")?.polyline_data(|p|{
                for a in v.iter(){
                    p.add_point(*a)?;
                }
                Ok(p)
            })
        })?;

        /*
        let svg=builder.attr("chicken","but")?.polyline_data(|p|{
            for a in v.iter(){
                p.add_point(*a)?;
            }
            Ok(p)
        })?.finish()?;
        */
        println!("{:?}",v);

        svg.elem2("div",|builder|{
            builder.build(|a|Ok(a))
        })?;
        


        svg.ok()
    })

}
