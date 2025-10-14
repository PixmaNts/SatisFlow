# Satifslow brief

Satisflow is a production tracker for player of the game satisfactory.

The purpose is to create a memory for the player of what is extracted (raw-input), what is produced (production line) and where (factory) and how items are exanged between facotry (logitics)

Satisflow is not an optimisation tools, it doesn't help to find the best recipe or calculate a train maximal output. Other tools availlable does exactly that.
The main goal here is to help the player keep an structured view of what is done, because, as the game progression advance, keeping memory of all the products we have availlable can become cumbersome.

## Factory

In the game, usualy, player create multiple factory or sub-factory around the game map.
Some of them can be specialized into some kind of production (Oil processing, Electronic Factory, ect ...) or even be dedicated to Energy production.
Some player can also choose to only have only one MegaFactory.

For tracking that model into satisflow, We create a Factory concept
User can of course do all CRUD operation on the FactoryData and subelement.

``` rust
pub struct Factory {
    pub id: FactoryId,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    pub raw_inputs: Vec<RawInput>,
    pub logistics_inputs: Vec<LogisticsInput>,
    pub production_lines: Vec<ProductionLine>,
    #[serde(default)]
    pub power_generators: Vec<PowerGenerator>,
}
```

A factory have an unique ID, a humain redeable name, metadatas desctription andnotes.
Then we have multiple element:

1. RawInput
2. LogisticsInput:
3. Productionlines
4. PowerGenerators

### Raw input

A raw inputs is every ressources (Item) that are gathered in the map like ores, water, oil, gaz.
Depending of the type of the Item extracted, we can have multiple form of RawInput:
Ore, Oil and Gaz have the concept of Purity (impure, normal, pure)
Water hoever doesn't.
Also, the extractor can change the amount of element extracted per/min (Miner MK1, Mk2, Mk3)

### LogisticInput

Logistics input are the items transported to the Factory. They provide information on the quantity/min (or m3/min in the case of gaz and liquid) and also the type of transportation (Conveyor, Pipeline, Train, Truck, Drone). They come from a LogisticLine (thant will be covered later on that document)

### ProductionLine

A production Line or the actual production in the factory.
It represent a group of machine that produce the same items (using a recipe).
As the game offer multiple way of tweaking the production (Overclocking, downcloking) or multiply the ouput using SomerSloop, the production line take into account theese specificity.
A player add a production line by :

1. selecting a factory,
2. selectring a recipe,
3. Defining one or multiple machine groups
4. For each machine groupe we can define an OC value from 0.000 to 250.000 that apply to all the machine of that groups. The precision here is important as the game offer such a precission.
5. The player can say if a group is equiped with 1,2,4 Somesloop for each machine in the group (depending on the machine type)
6. The player can then attach the production line to a meta group of production (eg: "Iron processing group")
7. The player can also add a comment for that particular production line (eg: "This line feed the Modular Frame production groupes")

### PowerGenerators

A PowerGenerators is like a production line, but there is no output item (execpt for nuclear energy)
The player should choose the type of carburant feeded into the generator depending on his type (eg: Fuel, TurboFuel, ect ...)
As production line, the power generator can be set with groups one or many.
Each group can have an OC value applyed to all the generator of that groups

## LogisticsLine

Satisflow help to track production with the factory concept, but also track how factory echange items.
Theyre is many way of making item move in the game:  

**Buses**:

- Support for multiple conveyors (Mk1-Mk6: 60-1200 items/min)
- Support for multiple pipelines (Mk1-Mk2: 300-600 m³/min)
- Mix conveyors and pipelines on the same bus
- Each item has unique ID (line_id for conveyors, pipeline_id for pipelines)

**Trains**:

- Support for multiple wagons
- Cargo wagons (solid items)
- Fluid wagons (fluids)
- Mix cargo and fluid wagons on the same train
- Each wagon has unique wagon_id

**Single-Item Transports**:

- Trucks (LG-TRK-###)
- Drones (LG-DRN-###)

A logistic line is defined like:

```Rust
pub struct LogisticsFlux {
    pub id: LogisticsFluxId,
    pub from_factory: FactoryId,
    pub to_factory: FactoryId,
    pub transport_type: TransportType,
    pub transport_details: String,
}
```

Where TransportType actualy contains the type of transport, and the detail for each type to resolve to the amount of item carryed and delivered as an input to a factory (and substraced from the source factory).

User can of course do all CRUD operation on the LogistricFlux data.

## The Engine

Satisflow engine, is writen in rust. The goal is to have a gui agnostic component that can then be used either with a native gui, a web interface (with a client/server model) or as a wasm module into a webapp, or even a mobile app if needed.
This lead to a very strong architecture decision:  **every rules, calculation, gamedata (recipe, item, coefficient( mk1-6), ect) should be defined in the engin, leveraging the Rust typestytem**

Feature:

- factory production calculation, power consuption and generation, defining product avaibility (underflow, overflow, balanced) ect ...
- persistence: every data can be serialized into json value, so that the user can actually save is progression on a json file, and reuse it later.
- engine should support blueprint (like in the game) as a custome recipe type.

### Custome Recipes

a custome recipe is define by

- the inputs items
- the ouputs items
- one or many subgroups of production line. (as production line)

Based on these information, the engine can calculate the energy consuption of the blueprint. A blueprint can't be OC or carrying multiplyer like somesloop. Hoever, the inner production line can be OC and multiplied.
The Custom recipe can be seen as a Particular type of ProductionLine, sharing multiple type of machine.

eg: I take 120 iron ingo in input and got 10 reinforced plate on output, using 5 Fabricator and 2 assembler.

## User interface

For the first version of the Satiflow tools, we chose to go with WASM and Vue.js (Typescript) to create a user interface.

The user interface give 3 view

A global Dashboard, a Factory View and a Logistics view.

### DashboardView

The Dashboard offer a global view of every item produced, the total power generation/consumption, the number of factory and the number of production line per factory, the number of logistics line.
It offer convenient filtering to see item source by factory name, a way to order by amount produced, groups by production line groups, selecting only Overflow, underflow, balanced item production.

### FactoryView

The Factory view display one factory at a time, selected by their name.
When a factory is selected, user have acces to 3 sub-tab : Production Line, RawInput, PowerGeneration.
In the production line subview, the user can see existing line, edit them, filter them by groups or items type, deletes them.
They can also create a new one. When creating a new production line, the user can acces 2 type of modal: one for creating a "classique recipe" production line, or a custome recipe one.
In the raw input sub-view, the use can add a raw input. It also see all existing raw input and can edit or delete them.
Same logic for the PowerGeneration sub-view.

### LogisticsView

The Logitics View allow the user to add/edit/delet and see every existing logistics line. He can also filter them by source, or destination or by type.
Multiple item logistics line should be grouped (Train and Bus).
When creating a new logistic line, in the case of Train and Bus, the user can choose to attach a new wagon or a new conveyor/pipeline to an existing train or buses (validating that the sources/destination are coherent). He can also choses to create a new one, even if there is already another one covering the same source/destination.

## Testing

Test should be written before the feature (TDD) to ensure code quality and feature completed.
On the engine side, test a mandatory for every aspect.
On the UI side, playwright is recommended for testing the UI.
