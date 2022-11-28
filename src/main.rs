use regex::Regex;

fn main() {
  // Number Validation Regex 
  let re = Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
  let mat = re.find("phone: 111-222-3333").unwrap();
  assert_eq!((mat.start(), mat.end()), (7, 19));
  println!("Number : {:?}", mat);

  // Username Validation Regex 
  // bhavik@gmail.com
  let username_regex = Regex::new(r"[a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?").unwrap();
  let username = "bhavik";
  let is_username_valid = username_regex.is_match("bhavik");
  println!("The Username {} is valid? {} ", username, is_username_valid);

  // Domain Validation Regex 
  // bhavik@gmail.com
  let domain_regex = Regex::new(r"^[a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6}$").unwrap();
  let domain = "xyz.com";
  let is_domain_valid = domain_regex.is_match(domain);
  println!("The Domain {} is valid? {}", domain, is_domain_valid);
  let domain = "xyz/com";
  let is_domain_valid = domain_regex.is_match(domain);
  println!("The Domain {} is valid? {}", domain, is_domain_valid);

  // Domain Validation Regex 
  // bhavik@gmail.com
  let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
  
  let email = "bhavik@gmail.com";
  let email1 = "bhavik@google.com";
  let email2 = "bhavik@sîeà.com";
  let email3 = "bhavik@soel1.com";
  
  let is_email_valid = email_regex.is_match(email);
  let is_email_valid1 = email_regex.is_match(email1);
  let is_email_valid2 = email_regex.is_match(email2);
  let is_email_valid3 = email_regex.is_match(email3);
  
  println!("The Email {} is valid? {} ", email, is_email_valid);
  println!("The Email {} is valid? {} ", email1, is_email_valid1);
  println!("The Email {} is valid? {} ", email2, is_email_valid2);
  println!("The Email {} is valid? {} ", email3, is_email_valid3);
}