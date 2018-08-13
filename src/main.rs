extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client, Filter};

fn main() {
    let client = Ec2Client::new(Region::ApNortheast1);

    let filter = Filter {
        name: Some("tag:Name".to_string()),
        values: Some(vec!["web".to_string()]),
    };

    let describe_instance_request = DescribeInstancesRequest {
        dry_run: Some(false),
        filters: Some(vec![filter]),
        instance_ids: None,
        max_results: None,
        next_token: None,
    };

    match client.describe_instances(describe_instance_request).sync() {
        Ok(output) => match output.reservations {
            Some(instances) => {
                println!("{:#?}", instances);
            }
            None => println!("No Instances"),
        },
        Err(error) => {
            println!("Error: {:#?}", error);
        }
    }
}
