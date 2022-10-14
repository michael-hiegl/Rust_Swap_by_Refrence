fn main()
{
    //Code###############################################
    let mut wert_1: i32 = 10;
    let mut wert_2: i32 = 20;

    print!("wert_1: {wert_1}, wert_2: {wert_2}\n");

    print!("Swap-Function is called now\n");
    swap(&mut wert_1, &mut wert_2);

    print!("wert_1: {wert_1}, wert_2: {wert_2}\n");
    //###################################################
}

//Functions##############################################
fn swap(a: &mut i32, b: &mut i32)
{
    let mut temp: i32 = 0;
    temp = *a;
    *a = *b;
    *b = temp;
}
//#######################################################