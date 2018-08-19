extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client, Filter, Instance, Reservation, Tag};

fn instance_list(reservations: Vec<Reservation>) -> Vec<Instance> {
    return reservations
        .into_iter()
        .flat_map(|reservation| reservation.instances)
        .flat_map(|v| v)
        .collect::<Vec<Instance>>();
}

fn tag_value(tag_list: Vec<Tag>) -> String {
    return tag_list
        .into_iter()
        .find(|tag| tag.key == Some("Name".to_string()))
        .map_or("".to_string(), |t| t.value.unwrap_or("".to_string()));
}

fn main() {
    let client = Ec2Client::new(Region::ApNortheast1);

    let filter = Filter {
        name: Some("instance-state-name".to_string()),
        values: Some(vec!["running".to_string()]),
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

                let ips = instances
                    .into_iter()
                    .map(|instance| {
                        let tag = tag_value(
                            instance
                                .tags
                                .into_iter()
                                .flat_map(|tags| tags)
                                .collect::<Vec<Tag>>(),
                        );
                        vec![instance.private_ip_address.unwrap(), tag]
                    })
                    .collect::<Vec<_>>();

                for ip in ips {
                    println!("{}\t{}", ip[0], ip[1]);
                }
            }
            None => println!("No Instances"),
        },
        Err(error) => {
            println!("Error: {:#?}", error);
        }
    }
}
