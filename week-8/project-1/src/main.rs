fn main() {

    let occupation = vec!["Office Administrator","Academic","Lawyer","Teacher"];

    println!("The following staff are to be validated: 1.{},2.{},3.{},4.{}",occupation[0],occupation[1],occupation[2],occupation[3]);

    let mut input1 = String::new();
    println!("Enter the serial number of your occupation(1,2,3 or 4): ");
    std::io::stdin().read_line(&mut input1).expect("Your occupation is not to be validated");
    let occ_no:i32 = input1.trim().parse().expect("Your occupation is not to be validated ");

    let mut oat_no = 0;
    let mut lt_no = 0;
    let mut at_no = 0;
    let mut tt_no = 0;

    if occ_no == 1 {
        let mut input2 = String::new();
        println!("You are an Office Administrator");
        let off_admin_type = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
        println!("Your occupation has the following levels: 1.{},2.{},3.{},4.{},5.{},6.{}",off_admin_type[0],off_admin_type[1],off_admin_type[2],off_admin_type[3],off_admin_type[4],off_admin_type[5]);
        std::io::stdin().read_line(&mut input2).expect("Your level isn't specified");
        let oat_no:i32 = input2.trim().parse().expect("Your level isn't specified");

    }
    else if occ_no == 2 {
        let mut input3 = String::new();
        println!("You are an Academic");
        let acad_type = vec!["Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];
        println!("Your occupation has the following levels: 1.{},2.{},3.{},4.{},5.{}",acad_type[0],acad_type[1],acad_type[2],acad_type[3],acad_type[4]);
        std::io::stdin().read_line(&mut input3).expect("Your level isn't specified");
        let at_no:i32 = input3.trim().parse().expect("Your level isn't specified");

    }else if occ_no == 3 {
         let mut input4 = String::new();
         println!("You are a Lawyer");
         let law_type = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];
         println!("Your occupation has the following levels: 1.{},2.{},3.{},4.{},5.{},6.{}",law_type[0],law_type[1],law_type[2],law_type[3],law_type[4],law_type[5]);
         std::io::stdin().read_line(&mut input4).expect("Your level isn't specified");
         let lt_no:i32 = input4.trim().parse().expect("Your level isn't specified");

    }else if occ_no == 4 {
        let mut input5 = String::new();
        println!("You are a Teacher");
        let tch_type = vec!["Placement","Classroom Teacher","Snr Teacher","Leading Teacher","Deputy Principal","Principal"];
        println!("Your occupation has the following levels: 1.{},2.{},3.{},4.{},5.{},6.{}",tch_type[0],tch_type[1],tch_type[2],tch_type[3],tch_type[4],tch_type[5]);
        std::io::stdin().read_line(&mut input5).expect("Your level isn't specified");
        let tt_no:i32 = input5.trim().parse().expect("Your level isn't specified");

    } else {
        println!("Not specified,");
    }
    if oat_no == 1 || lt_no == 1 || tt_no == 1 {
        println!("Your Staff Level is APS 1-2");
    }
    else if oat_no == 2 || at_no == 2 || tt_no == 2 || lt_no ==2 {
        println!("Your Staff Level is APS 3-5");
    }
    else if oat_no == 3 || at_no == 3 || lt_no == 3 || at_no == 3 {
        println!("Your Staff Level is APS 5-8");
    }
    else if oat_no == 4 || at_no == 4 || lt_no == 4 || at_no == 4 {
        println!("Your Staff Level is EL1 8-10");
    }
    else if oat_no == 5 && at_no == 5 && lt_no == 5 && at_no == 5 {
        println!("Your Staff Level is EL2 10-13");
    }
    else if oat_no == 6 && at_no == 6 && lt_no == 6 && at_no == 6 {
        println!("Your Staff Level is SES");
    } else {
        println!("Unspecified Staff Level");
    }






}
