## The christmas miracle
I'm currently travelling home for christmas, and like everytime, I'm excited to see the family,
but also thinking what will I do in the downtime I have. There will certainly be a few hours left that I will 
want to code.

So, considering this free time, I've decided to bring you a christmas miracle I was wishing for some time :)

## Android Binder
Great pieces of software for android, Lofty original goals, etc ...
Works relatively great ( few security issues over the years). Is missing a proper rust binding.
Could be great if extended with hwbinder support (Binderized HALs in rust would be the ultimate end)

## Goals.
1. Write binder library, allowing to create a simple native service that accepts remote transactions.
	- Low level binder code
	- Definition of service & stub traits
	- Runtime support ( register service, get service from manger, transact )

2. Write parcelable library, which will have custom derive macro to derive parcelable impl on structs.
	- Native data types (scalars + strings - utf16 conversion, FDs, )

3. Write AIDL parser what will read AIDL definition, and generate necessary stuff
	- Trait definition for services
	- Service stub impl
	- Service host impl (stuff tat dispatches to trait impl of actual service)

4. Generate code for common android services
	- Allow writing completely native services (no VM, low mem)


## Implementation goals
1. Low-overhead - Allow the use of the library to have android on very embedded stuff
2. Simplicity - New service should be these steps:
	1. Write AIDL
	2. Implement service trait from AIDL
	3. Create bare APK (no java stuffs) - Other libs


