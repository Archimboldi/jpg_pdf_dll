extern crate libloading;
extern crate libc;
extern crate encoding;
use std::fs;
use std::env;
use std::os::windows::fs::symlink_file;
use std::collections::HashMap;

use std::ffi::CString;
use libc::*;
use libloading::{Library,Symbol};
use encoding::all::GBK;
use encoding::{Encoding,EncoderTrap};

struct PDF;
type New=unsafe fn()->*const PDF;
type BeginDoc=unsafe fn(*const PDF,*const c_char,c_int,*const c_char)->i32;
type LoadImage=unsafe fn(*const PDF,*const c_char,*const c_char,c_int,*const c_char)->i32;
type Infoimage=unsafe fn(*const PDF,c_int,*const c_char,*const c_char)->c_double;
type BeginPage=unsafe fn(*const PDF,c_double,c_double,*const c_char);
type FitImage=unsafe fn(*const PDF,c_int,c_double,c_double,*const c_char);
type CloseImage=unsafe fn(*const PDF,c_int);
type EndPage=unsafe fn(*const PDF,*const c_char);
type CreateBookmark =unsafe fn(*const PDF,*const c_char,c_int,*const c_char)->i32;
type EndDoc=unsafe fn(*const PDF,*const c_char);

fn main(){
    let mut arguments = Vec::new();
    let mut n=0;
    for argument in env::args() {
        arguments.push(argument);
        n+=1;
    }

    if arguments[1]!="" && arguments[2]!=""{
  
        let mut a=env::current_dir().unwrap();
        a.push("pdflib.dll");

        let lib=Library::new(a).unwrap();
        let cd=&arguments[1];
        let filn=&arguments[2];

        let mut shi:HashMap<i32,&str>=HashMap::new();
        shi.insert(1, "一、履历类材料");
        shi.insert(2, "二、自传和思想类材料");
        shi.insert(3, "三、考核鉴定类材料");
        shi.insert(4, "四、学历学位、专业技术职务（职称）、学术评鉴和教育培训类材料");
        shi.insert(5, "五、政审、审计和审核类材料");
        shi.insert(6, "六、党、团类材料");
        shi.insert(7, "七、表彰奖励类材料");
        shi.insert(8, "八、违规违纪违法处理处分类材料");
        shi.insert(9, "九、工资、任免、出国和会议代表类材料");
        shi.insert(10, "十、其他可供组织参考的材料");
        
        if let Ok(dirs) = fs::read_dir(cd) {
            //人
            for path in dirs{
                unsafe{
                    let pdf_new:Symbol<New>=lib.get(b"PDF_new").unwrap();
                    let pdf_begin_document:Symbol<BeginDoc>=lib.get(b"PDF_begin_document").unwrap();
                    let pdf_load_image:Symbol<LoadImage>=lib.get(b"PDF_load_image").unwrap();
                    let pdf_info_image:Symbol<Infoimage>=lib.get(b"PDF_info_image").unwrap();
                    let pdf_begin_page_ext:Symbol<BeginPage>=lib.get(b"PDF_begin_page_ext").unwrap();
                    let pdf_fit_image:Symbol<FitImage>=lib.get(b"PDF_fit_image").unwrap();
                    let pdf_close_image:Symbol<CloseImage>=lib.get(b"PDF_close_image").unwrap();
                    let pdf_end_page_ext:Symbol<EndPage>=lib.get(b"PDF_end_page_ext").unwrap();
                    let pdf_create_bookmark:Symbol<CreateBookmark>=lib.get(b"PDF_create_bookmark").unwrap();
                    let pdf_end_document:Symbol<EndDoc>=lib.get(b"PDF_end_document").unwrap();

                    if let Ok(path) = path {
                        if path.path().is_dir(){

                            let p=pdf_new();
                            let filename=CString::new("temp.pdf").unwrap();
                            let imagetype=CString::new("jpeg").unwrap();
                            let iw=CString::new("imagewidth").unwrap();
                            let ih=CString::new("imageheight").unwrap();
                            let opt=CString::new("").unwrap();

                            if n>4{
                                let psw=CString::new(format!("{} {}",&arguments[3],&arguments[4])).unwrap();
                                pdf_begin_document(p,filename.as_ptr(),0,psw.as_ptr());
                            }else{pdf_begin_document(p,filename.as_ptr(),0,opt.as_ptr());}
                            
                            if let Ok(dir) = fs::read_dir(path.path()) {
                                //十
                                let mut i=1;
                                for _ in dir{
                                    let mut pat=String::from(path.path().to_str().unwrap());
                                    pat.push('/');
                                    pat.push_str(shi.get(&i).unwrap());
                                    
                                    //bookmark
                                    let bookmark=CString::new(GBK.encode(shi.get(&i).unwrap(),EncoderTrap::Strict).unwrap()).unwrap();
                                    let u= pdf_create_bookmark(p,bookmark.as_ptr(),0,opt.as_ptr());

                                    if let Ok(di) = fs::read_dir(pat) {
                                        
                                        //一
                                        for pa in di{
                                            if let Ok(pa) = pa {

                                                if pa.path().is_dir(){
                                                    if let Ok(d) = fs::read_dir(pa.path()) {
                                                        //2                                                           
                                                        let f=CString::new(format!("parent={}",u)).unwrap();
                                                        //bookmark
                                                        let bookmark=CString::new(GBK.encode(pa.file_name().to_str().unwrap(),EncoderTrap::Strict).unwrap()).unwrap();
                                                        let u2= pdf_create_bookmark(p,bookmark.as_ptr(),0,f.as_ptr());
                                                        for a in d{
                                                            if let Ok(a) = a {
                                                                
                                                                //image
                                                                let iname=a.file_name();
                                                                let sc:Vec<&str>=iname.to_str().unwrap().rsplit(".").collect();
                                                                if sc[0]=="jpg"{
                                                                    symlink_file(a.path(),&iname).unwrap();
                                                                    let imagename=CString::new(iname.to_str().unwrap()).unwrap();
                                                                    let j=pdf_load_image(p,imagetype.as_ptr(),imagename.as_ptr(),0,opt.as_ptr());
                                                                    fs::remove_file(a.file_name()).unwrap();
                                                                    
                                                                    let w= pdf_info_image(p,j,iw.as_ptr(),opt.as_ptr());
                                                                    let h=pdf_info_image(p,j,ih.as_ptr(),opt.as_ptr());
                                                                    if h>w{
                                                                        pdf_begin_page_ext(p,595.0,842.0,opt.as_ptr());
                                                                        let adj=CString::new("boxsize={595 842} position=center fitmethod=meet").unwrap();
                                                                        pdf_fit_image(p,j,0.0,0.0,adj.as_ptr());
                                                                        pdf_close_image(p,j);
                                                                    }else{
                                                                        pdf_begin_page_ext(p,842.0,595.0,opt.as_ptr());
                                                                        let adj=CString::new("boxsize={842 595} position=center fitmethod=meet").unwrap();
                                                                        pdf_fit_image(p,j,0.0,0.0,adj.as_ptr());
                                                                        pdf_close_image(p,j);
                                                                    }
                                                                    
                                                                    let f2=CString::new(format!("parent={}",u2)).unwrap();
                                                                    //bookmark
                                                                    let bookmark=CString::new(GBK.encode(sc[1],EncoderTrap::Strict).unwrap()).unwrap();
                                                                    pdf_create_bookmark(p,bookmark.as_ptr(),0,f2.as_ptr());
                                                                    pdf_end_page_ext(p,opt.as_ptr());
                                                                }
                                                            }
                                                        }    
                                                    }
                                                    
                                                }else{

                                                    //image
                                                    let iname=pa.file_name();
                                                    let sc:Vec<&str>=iname.to_str().unwrap().rsplit(".").collect();
                                                    if sc[0]=="jpg"{
                                                        symlink_file(pa.path(),&iname).unwrap();
                                                        let imagename=CString::new(iname.to_str().unwrap()).unwrap();
                                                        let j=pdf_load_image(p,imagetype.as_ptr(),imagename.as_ptr(),0,opt.as_ptr());
                                                        fs::remove_file(pa.file_name()).unwrap();
                                                        
                                                        let w= pdf_info_image(p,j,iw.as_ptr(),opt.as_ptr());
                                                        let h=pdf_info_image(p,j,ih.as_ptr(),opt.as_ptr());
                                                        if h>w{
                                                            pdf_begin_page_ext(p,595.0,842.0,opt.as_ptr());
                                                            let adj=CString::new("boxsize={595 842} position=center fitmethod=meet").unwrap();
                                                            pdf_fit_image(p,j,0.0,0.0,adj.as_ptr());
                                                            pdf_close_image(p,j);
                                                        }else{
                                                            pdf_begin_page_ext(p,842.0,595.0,opt.as_ptr());
                                                            let adj=CString::new("boxsize={842 595} position=center fitmethod=meet").unwrap();
                                                            pdf_fit_image(p,j,0.0,0.0,adj.as_ptr());
                                                            pdf_close_image(p,j);
                                                        }

                                                        let f=CString::new(format!("parent={}",u)).unwrap();
                                                        //bookmark
                                                        let bookmark=CString::new(GBK.encode(sc[1],EncoderTrap::Strict).unwrap()).unwrap();
                                                        pdf_create_bookmark(p,bookmark.as_ptr(),0,f.as_ptr());

                                                        pdf_end_page_ext(p,opt.as_ptr());
                                                    }
                                                    

                                                }                                   
                                            }
                                        }
                                        
                                        
                                    }
                                    i+=1;
                                }
                            }
                            pdf_end_document(p,opt.as_ptr());
                            let filn=format!("{}{}.pdf",filn,path.file_name().to_str().unwrap());
                            fs::copy("temp.pdf", filn).unwrap();
                            fs::remove_file("temp.pdf").unwrap();
                        }
                    }
                }
            }
        }
    }
}