use crate::hardware::CPUInfo;
use regex::Regex;

fn get_cpu_model() -> (String, String, String) {
    let cpu_info = std::fs::read_to_string("/proc/cpuinfo").expect("Unable to find /proc/cpuinfo");

    let regex_err = "CPU Regex Compile Failure";
    let family_regex = Regex::new(r#"cpu family\s*:\s(.*)"#).expect(regex_err);
    let model_regex = Regex::new(r#"model\s*:\s(.*)"#).expect(regex_err);
    let model_name_regex = Regex::new(r#"model name\s*:\s(.*)"#).expect(regex_err);

    let family = family_regex
        .captures(&cpu_info)
        .expect("Unable to find cpu family in /proc/cpuinfo")
        .get(1)
        .expect("Missing first capture group in cpu regex")
        .as_str()
        .to_string();

    let model = model_regex
        .captures(&cpu_info)
        .expect("Unable to find cpu model in /proc/cpuinfo")
        .get(1)
        .expect("Missing first capture group in cpu regex")
        .as_str()
        .to_string();

    let model_name = model_name_regex
        .captures(&cpu_info)
        .expect("Unable to find model name in /proc/cpuinfo")
        .get(1)
        .expect("Missing first capture group in cpu regex")
        .as_str()
        .to_string();

    (family, model, model_name)
}

fn get_memory_info() -> usize {
    let mem_info = std::fs::read_to_string("/proc/meminfo").expect("Unable to find /proc/meminfo");

    let regex_err = "Memory Regex Compile Failure";
    let memory_regex = Regex::new(r#"MemTotal:\s*(.*?) kB"#).expect(regex_err);

    let memory = memory_regex
        .captures(&mem_info)
        .expect("Unable to find memory total in /proc/meminfo")
        .get(1)
        .expect("Missing first capture group in memory regex")
        .as_str()
        .parse::<usize>()
        .expect("Memory size in /proc/meminfo is not a number")
        * 1024;

    memory
}

pub fn get_cpu() -> CPUInfo {
    let (family, model, model_name) = get_cpu_model();
    let memory = get_memory_info();
    let cores = num_cpus::get();

    CPUInfo {
        family,
        model,
        model_name,
        memory,
        cores,
    }
}
