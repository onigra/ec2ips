extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client, Filter, Instance, Reservation};

fn instance_list(reservations: Vec<Reservation>) -> Vec<Instance> {
    let list = reservations
        .into_iter()
        .flat_map(|reservation| reservation.instances)
        .collect::<Vec<_>>();

    return list.into_iter().flat_map(|v| v).collect::<Vec<Instance>>();
}

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
        Ok(response) => match response.reservations {
            Some(reservations) => {
                let instances = instance_list(reservations);

                let private_ips = instances
                    .into_iter()
                    .flat_map(|instance| instance.private_ip_address)
                    .collect::<Vec<String>>();

                for ip in private_ips {
                    println!("{}", ip)
                }
            }
            None => println!("No Instances"),
        },
        Err(error) => {
            println!("Error: {:#?}", error);
        }
    }
}
