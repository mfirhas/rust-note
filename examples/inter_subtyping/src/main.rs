mod closure_function_pointer;
mod lifetime_st;
mod trait_obj_st;
mod trait_st;
mod variances;

fn main() {
    trait_st::do_trait();
    lifetime_st::lifetime_subtyping();
    closure_function_pointer::f_a();
    trait_obj_st::trait_object();
}
