use std::io::Write;

fn main(){
    //welcome address
    println!("\nWELCOME!
    \nErnest and Young Global Limited Database.");
    println!("\nWhich of our staff Records would you like to access?");
    
    println!("\nEnter '7' if you would like to access the files of Mrs.Aigbona Juliet & Mr.Akpevwe Iloka ");
    println!("Enter '8' if you would like to access the files of Mr.Adamu Sagamu & Mr.Gbenga Daniels ");
    println!("Enter '9' if you would like to access the files of Mr.Ehis Ero & Mrs.Maria Akinsola \n");
    println!("Enter an option:");
    let mut _input1 = String::new();
    std::io::stdin().read_line(&mut _input1).expect("Invalid input");
    let _input1:i32 = _input1.trim().parse().expect("Invalid input");


// calling the functions
if _input1 == 7{
    code_7()
    }
    else if _input1 == 8{
    code_8()
    }
    else if _input1 == 9{
    code_9()
    }
    else {
        println!{"Invalid input"}

}
    //Creating the functions for code_7

            fn code_7(){ 

        println!("Whose file would you like to access?");
        println!("Enter '1' for Mrs.Aigbona Juliet OR Enter '2' for Mr.Akpevwe Iloka ");
        let mut file_create = String::new();
        std::io::stdin().read_line(&mut file_create).expect("Invalid input");
        let file_create:i32 = file_create.trim().parse().expect("Invalid input");

        if file_create == 1{
            let name = "Aigbona Juliet";
            let dept = "Consulting ";
            let services = {"Analytics consulting services, Customer experience, Cybersecurity, strategy,risk, compliance and resilience,\n Digital transformation,Risk consulting services, Supply chain and operations, Technology transformation" };

    let quali = "B.Sc.";
    let mut file = std::fs::File::create("Aigbona_Juliet.txt").expect("create failed");
    file.write_all(name.as_bytes()).expect("write failed");
    file.write_all("\nDepartment: ".as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    file.write_all("\nQualification: ".as_bytes()).expect("write failed");
    file.write_all(quali.as_bytes()).expect("write failed");
    file.write_all("\nCode:  7".as_bytes()).expect("write failed");
    file.write_all("\nServices: ".as_bytes()).expect("write failed");
    file.write_all(services.as_bytes()).expect("write failed");
    println!("\nThe file has been created.");
    println!("Visit the Folder to acess the staff file.");
    }
    else if file_create == 2{
    let name = "Akpevwe Iloka";
    let dept = "Assurance";
    let services = {"Audit services, Climate change and sustainability services, Financial accounting advisory services,\nForensic and integrity services, Private client audit experience, Accounting Link, Assurance"};
    let quali = "HND";
    let mut file = std::fs::File::create("Akpevwe_Iloka.txt").expect("create failed");
    file.write_all(name.as_bytes()).expect("write failed");
    file.write_all("\nDepartment: ".as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    file.write_all("\nQualification: ".as_bytes()).expect("write failed");
    file.write_all(quali.as_bytes()).expect("write failed");
    file.write_all("\nCode:  7".as_bytes()).expect("write failed");
    file.write_all("\nServices: ".as_bytes()).expect("write failed");
    file.write_all(services.as_bytes()).expect("write failed");
    println!("\nThe file has been created.");
    println!("Visit the Folder to acess the staff file.");
    }
    else {
        println!{"Invalid input"}
    }
            
        }

               fn code_8() {
                println!("Whose file would you like to access?");
                println!("Enter '1' for Mr.Adamu Sagamu OR Enter '2' for Mr.Gbenga Daniels ");
            let mut file_create = String::new();
            std::io::stdin().read_line(&mut file_create).expect("Invalid input");
            let file_create:i32 = file_create.trim().parse().expect("Invalid input");
        
            if file_create == 1 {  
            let name = "Adamu Sagamu";
            let dept = "Tax";
            let services = {"Tax planning, Tax function operations, Tax policy and controversy,\nGlobal trade, Tax accounting, Tax compliance, Transaction tax"};
            let quali = "B.Sc.";
            let mut file = std::fs::File::create("Adamu_Sagamu.txt").expect("create failed");
            file.write_all(name.as_bytes()).expect("write failed");
            file.write_all("\nDepartment: ".as_bytes()).expect("write failed");
            file.write_all(dept.as_bytes()).expect("write failed");
            file.write_all("\nQualification: ".as_bytes()).expect("write failed");
            file.write_all(quali.as_bytes()).expect("write failed");
            file.write_all("\nCode:  8".as_bytes()).expect("write failed");
            file.write_all("\nServices: ".as_bytes()).expect("write failed");
            file.write_all(services.as_bytes()).expect("write failed");
            println!("\nThe file has been created.");
            println!("Visit the Folder to acess the staff file.");
            }
            else if file_create == 2{
            let name = "Gbenga Daniels";
            let dept = "People and workforce";
            let services = {"Change management and experience, HR transformation, Integrated workforce mobility, Learning and  development consulting,\nRecognition and reward advisory,Workforce analytics, People and workforce"};
            let quali = "HND";
            let mut file = std::fs::File::create("Gbenga_Daniels.txt").expect("create failed");
            file.write_all(name.as_bytes()).expect("write failed");
            file.write_all("\nDepartment: ".as_bytes()).expect("write failed");
            file.write_all(dept.as_bytes()).expect("write failed");
            file.write_all("\nQualification: ".as_bytes()).expect("write failed");
            file.write_all(quali.as_bytes()).expect("write failed");
            file.write_all("\nCode:  8".as_bytes()).expect("write failed");
            file.write_all("\nServices: ".as_bytes()).expect("write failed");
            file.write_all(services.as_bytes()).expect("write failed");
            println!("\nThe file has been created.");
            println!("Visit the Folder to acess the staff file.");
            }
            else {
                println!{"Invalid input"}
            }
        }

        fn code_9() {
            println!("Whose file would you like to access?");
            println!("Enter '1' for Mr.Ehis Ero OR Enter '2' for Mrs.Maria Akinsola ");
            let mut file_create = String::new();
            std::io::stdin().read_line(&mut file_create).expect("Invalid input");
            let file_create:i32 = file_create.trim().parse().expect("Invalid input");
        
            if file_create == 1 {  
            let name = "Ehis Ero";
            let dept = "Strategy";
            let services = {"Strategy consulting, Corporate and growth strategy, Transaction strategy and execution,\nRestructuring and turnaround strategy, Industry strategy, Digital business building, Commercial strategy"};
            let quali = "M.Sc.";
            let mut file = std::fs::File::create("Ehis_Ero.txt").expect("create failed");
            file.write_all(name.as_bytes()).expect("write failed");
            file.write_all("\nDepartment: ".as_bytes()).expect("write failed");
            file.write_all(dept.as_bytes()).expect("write failed");
            file.write_all("\nQualification: ".as_bytes()).expect("write failed");
            file.write_all(quali.as_bytes()).expect("write failed");
            file.write_all("\nCode:  9".as_bytes()).expect("write failed");
            file.write_all("\nServices: ".as_bytes()).expect("write failed");
            file.write_all(services.as_bytes()).expect("write failed");
            println!("\nThe file has been created.");
            println!("Visit the Folder to acess the staff file.");
            }
            else if file_create == 2{
            let name = "Maria Akinsola";
            let dept = "Transactions and corporate finance";
            let services = {"Corporate finance,Divestments and carve-outs, Sustainability and ESG Services,\nM&A advisory, M&A integration, M&A technology and tools, M&A advanced analytics"};
            let quali = "M.Sc.";
            let mut file = std::fs::File::create("Maria_Akinsola.txt").expect("create failed");
            file.write_all(name.as_bytes()).expect("write failed");
            file.write_all("\nDepartment: ".as_bytes()).expect("write failed");
            file.write_all(dept.as_bytes()).expect("write failed");
            file.write_all("\nQualification: ".as_bytes()).expect("write failed");
            file.write_all(quali.as_bytes()).expect("write failed");
            file.write_all("\nCode:  9".as_bytes()).expect("write failed");
            file.write_all("\nServices: ".as_bytes()).expect("write failed");
            file.write_all(services.as_bytes()).expect("write failed");
            println!("\nThe file has been created.");
            println!("Visit the Folder to acess the staff file.");
            }
            else {
                println!{"Invalid input"}
            }
        }
    }
