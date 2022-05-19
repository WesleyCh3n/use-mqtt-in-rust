# Use MQTT in rust to do stuff

pub: control/cmd
pub: control/<node_number>/cmd  payload: cmd
sub: control/cmd/output

pub: prog/<program_name>/       payload: option{start, kill}
sub: prog/<program_name>/output

pub: ota/<program_name>/update/<version>
sub: ota/<program_name>/update/output
