extern crate specs;
extern crate nameof;

use specs::prelude::*;
use nameof::*;

#[derive(Debug, Copy, Clone, Default)]
struct Vec2(f32, f32);

#[derive(Debug, Copy, Clone, Default)]
struct Size
{
    pub width: Option<f32>,
    pub height: Option<f32>,
}
#[derive(Debug, Copy, Clone, Default)]
struct Rect
{
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Alignment
{
    Front, // left/top
    Middle, // center
    Back, // right/bottom

    Stretch, //special case
}

// A component contains data which is associated with an entity.

#[derive(Debug, Clone)]
struct UIElement {
    parent: Option<Entity>,

    //name: &str,

    //enabled/visible?

    //modal
    
    //layout - separate?
    position: Vec2,
    size: Size,
    padding: Vec2,
    horizontal_alignment: Alignment,
    vertical_alignment: Alignment,

    measured_size: Vec2,
    content_area: Rect,
    offset_content_area: Rect,
    visible_content_area: Rect,
    visible_bounds: Rect,

    last_measure_available_size: Vec2,
    last_measure_container_bounds: Rect,
    //todo: getters for all
}
impl UIElement {
    pub fn new() -> Self
    {
        return UIElement {
            parent: None,

            position: Vec2::default(),
            size: Size::default(),
            padding: Vec2::default(),

            horizontal_alignment: Alignment::Front,
            vertical_alignment: Alignment::Front,

            measured_size: Vec2::default(),
            content_area: Rect::default(),
            offset_content_area: Rect::default(),
            visible_content_area: Rect::default(),
            visible_bounds: Rect::default(),
            
            last_measure_available_size: Vec2(std::f32::INFINITY, std::f32::INFINITY), //may not be necessary
            last_measure_container_bounds: Rect::default(),
        };
    }

    //visible_offset
    pub fn visible_offset(&self) -> Vec2 {
        return Vec2( 
            self.visible_content_area.x - self.offset_content_area.x,
            self.visible_content_area.y - self.offset_content_area.y
        );
    }
}
impl UIElement
{
    pub fn measure(&self, mut available_size: Vec2) -> Vec2
    {
        //if (isMeasureValid && availableSize == lastMeasureAvailableSize)
        //  return MeasuredSize;

        //self.last_measure_available_size = available_size;
        
        let mut size = Vec2(
            self.size.width.unwrap_or_default(),
            self.size.height.unwrap_or_default()
        );

        let is_width_auto_sized = self.size.width.is_none();
        let is_height_auto_sized = self.size.height.is_none();
        let is_h_stretch = self.horizontal_alignment == Alignment::Stretch;
        let is_v_stretch = self.vertical_alignment == Alignment::Stretch;

        if available_size.0 < std::f32::INFINITY
        {
            available_size.0 -= self.padding.0 * 2.0;
            if is_width_auto_sized && is_h_stretch
            {
                available_size.0 -= self.position.0; //round?
            }
        }
        else if !is_width_auto_sized
        {
            available_size.0 = size.0;
        }

        if available_size.1 < std::f32::INFINITY
        {
            available_size.1 -= self.padding.1 * 2.0;
            if is_height_auto_sized && is_v_stretch
            {
                available_size.1 -= self.position.1; //round?
            }
        }
        else if !is_height_auto_sized
        {
            available_size.1 = size.1;
        }

        //let measuredSize = measure_override(available_size);
        let measured_size = Vec2::default(); //todo
        if is_width_auto_sized || is_height_auto_sized
        {
            assert!(measured_size.0.is_finite());
            assert!(measured_size.1.is_finite());

            if is_width_auto_sized
            {
                size.0 = measured_size.0; //stretched items do have intrinsic size
                                          //size.0 = is_h_stretch ? 0 : measured_size.0; //stretched items have no intrinsic size
            }

            if is_height_auto_sized
            {
                size.1 = measured_size.1; //stretched items do have intrinsic size
                                          //size.1 = is_v_stretch ? 0 : measured_size.Y; //stretched items have no intrinsic size
            }
        }

        let final_size = Vec2(
            self.position.0 + size.0 + self.padding.0 * 2.0,
            self.position.1 + size.1 + self.padding.1 * 2.0
        );

        //isMeasureValid = true;
        //if final_size != measured_size
        //{
        //    invalidate arrange
        //    NotifyParentMeasuredSizeChanged();
        //}

        return final_size;
    }

    fn get_aligned_position(alignment: Alignment, position: f32, size: f32, padding: f32, container_size: f32) -> f32
    {
        match alignment {
            // stretched items will either fill full area or center in available space
            Alignment::Middle | Alignment::Stretch => (container_size - size + padding * 2.0) / 2.0 + position,
            Alignment::Back => container_size - size, //size includes padding and position
            _ => position + padding
        }
    }

    fn adjust_to_container(&self, container: Rect)
    {

    }

    pub fn arrange(&self, container: Rect)
    {
        self.adjust_to_container(container);
        // arrange override
    }
}
impl Component for UIElement {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

#[derive(Debug, Default)]
struct UIElementNeedsMeasure;
impl Component for UIElementNeedsMeasure {
    type Storage = specs::storage::NullStorage<Self>;
}

#[derive(Debug, Default)]
struct UIElementNeedsArrange;
impl Component for UIElementNeedsArrange {
    type Storage = specs::storage::NullStorage<Self>;
}

struct UILayout;
impl<'a> System<'a> for UILayout {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, UIElement>,
        WriteStorage<'a, UIElementNeedsMeasure>,
        WriteStorage<'a, UIElementNeedsArrange>,
    );
    
    fn run(&mut self, (ents, elems, mut measures, mut arranges): Self::SystemData) {
        for (ent, elem, _measure) in (&*ents, &elems, &measures).join() {
            println!("measure {:?}", ent); 
            elem.measure(elem.last_measure_available_size);
            //measure
        }
        measures.clear();
        //can data change between these?
        for (ent, elem, _arrange) in (&*ents, &elems, &arranges).join() {
            println!("arrange {:?}", ent); 

            elem.arrange(elem.last_measure_container_bounds);
        }
        arranges.clear();

        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        // You could also use `par_join()` to get a rayon `ParallelIterator`.
        for (ent, elem) in (&*ents, &elems).join() {
            println!("{:?}", ent.id());

            //good way to trigger dirty externally (events?)
        }
    }
}

fn main() {
    // The `World` is our
    // container for components
    // and other resources.
    
    let mut world = World::new();
    
    // This builds a dispatcher.
    // The third parameter of `add` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    let mut dispatcher = DispatcherBuilder::new().with(UILayout, name_of_type!(UILayout), &[]).build();
    
    // setup() must be called before creating any entity, it will register
    // all Components and Resources that Systems depend on
    dispatcher.setup(&mut world);
    
    let ent = world.create_entity().with(UIElement::new()).with(UIElementNeedsMeasure::default()).build();
        
    // This dispatches all the systems in parallel (but blocking).
    // loop
    {
        dispatcher.dispatch(&world);
        println!("---");
        dispatcher.dispatch(&world);
        println!("---");
        dispatcher.dispatch(&world);
    }

        println!("===");
        println!("{:#?}", ent.id());
}

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn test_bad_add() {
//         // This assert would fire and test will fail.
//         // Please note, that private functions can be tested too!
//         assert_eq!(bad_add(1, 2), 3);
//     }
// }